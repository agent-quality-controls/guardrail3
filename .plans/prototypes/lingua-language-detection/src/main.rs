use lingua::{Language, LanguageDetector, LanguageDetectorBuilder};

const RELIABLE_CONFIDENCE: f64 = 0.85;
const RELIABLE_MARGIN: f64 = 0.15;

const LANGUAGES: [Language; 7] = [
    Language::English,
    Language::Russian,
    Language::German,
    Language::French,
    Language::Spanish,
    Language::Portuguese,
    Language::Indonesian,
];

struct Sample {
    name: &'static str,
    kind: SampleKind,
    expected: Option<Language>,
    route_language: Option<Language>,
    ui_language: Option<Language>,
    body_language: Option<Language>,
    text: &'static str,
    ui_text: Option<&'static str>,
    body_text: Option<&'static str>,
}

#[derive(Clone, Copy)]
enum SampleKind {
    ShortUi,
    Article,
    MixedPage,
    Ambiguous,
}

fn main() {
    let detector = LanguageDetectorBuilder::from_languages(&LANGUAGES).build();

    println!("Lingua language detection prototype");
    println!("Restricted languages: {}\n", language_list());

    for sample in samples() {
        print_sample(&detector, &sample);
    }
}

fn print_sample(detector: &LanguageDetector, sample: &Sample) {
    println!("== {} ==", sample.name);
    println!("kind: {}", sample.kind.name());
    if let Some(expected) = sample.expected {
        println!("expected: {}", expected);
    }
    if let Some(route_language) = sample.route_language {
        println!("route language: {}", route_language);
    }
    if let Some(ui_language) = sample.ui_language {
        println!("ui language: {}", ui_language);
    }
    if let Some(body_language) = sample.body_language {
        println!("expected body language: {}", body_language);
    }

    print_detection(detector, "full text", sample.text);

    if let Some(ui_text) = sample.ui_text {
        print_detection(detector, "ui text", ui_text);
    }

    if let Some(body_text) = sample.body_text {
        print_detection(detector, "body text", body_text);
    }

    if let (Some(route_language), Some(body_text)) = (sample.route_language, sample.body_text) {
        if let Some(body_detected) = detector.detect_language_of(body_text) {
            if route_language != body_detected {
                println!(
                    "mismatch: route={} body_detected={}",
                    route_language, body_detected
                );
            } else {
                println!("mismatch: none");
            }
        }
    }

    let spans = detector.detect_multiple_languages_of(sample.text);
    if spans.len() > 1 {
        println!("multi-language spans:");
        for span in spans {
            println!(
                "  {} [{}..{}]: {}",
                span.language(),
                span.start_index(),
                span.end_index(),
                sample
                    .text
                    .get(span.start_index()..span.end_index())
                    .unwrap_or("")
                    .replace('\n', " ")
            );
        }
    }

    println!();
}

fn print_detection(detector: &LanguageDetector, label: &str, text: &str) {
    let confidence_values = detector.compute_language_confidence_values(text);
    let detection = Detection::from_confidence_values(confidence_values.as_slice());

    println!(
        "{} detected: {}",
        label,
        display_language(detection.language)
    );
    println!(
        "{} decision: {}",
        label,
        if detection.is_reliable() {
            "reliable"
        } else {
            "ambiguous"
        }
    );
    print!("{} confidence:", label);
    for (language, confidence) in confidence_values {
        print!(" {}={:.3}", language, confidence);
    }
    println!();
}

fn display_language(language: Option<Language>) -> String {
    language.map_or_else(|| "unknown".to_owned(), |language| language.to_string())
}

struct Detection {
    language: Option<Language>,
    confidence: f64,
    runner_up_confidence: f64,
}

impl Detection {
    fn from_confidence_values(confidence_values: &[(Language, f64)]) -> Self {
        let mut values = confidence_values.to_vec();
        values.sort_by(|left, right| right.1.total_cmp(&left.1));

        let Some((language, confidence)) = values.first().copied() else {
            return Self {
                language: None,
                confidence: 0.0,
                runner_up_confidence: 0.0,
            };
        };

        Self {
            language: Some(language),
            confidence,
            runner_up_confidence: values.get(1).map_or(0.0, |(_, confidence)| *confidence),
        }
    }

    fn is_reliable(&self) -> bool {
        self.confidence >= RELIABLE_CONFIDENCE
            && self.confidence - self.runner_up_confidence >= RELIABLE_MARGIN
    }
}

fn language_list() -> String {
    LANGUAGES
        .iter()
        .map(ToString::to_string)
        .collect::<Vec<_>>()
        .join(", ")
}

impl SampleKind {
    fn name(self) -> &'static str {
        match self {
            Self::ShortUi => "short-ui",
            Self::Article => "article",
            Self::MixedPage => "mixed-page",
            Self::Ambiguous => "ambiguous",
        }
    }
}

fn samples() -> Vec<Sample> {
    vec![
        Sample {
            name: "en short UI",
            kind: SampleKind::ShortUi,
            expected: Some(Language::English),
            route_language: None,
            ui_language: None,
            body_language: None,
            text: "Read more Subscribe Back Next Search",
            ui_text: None,
            body_text: None,
        },
        Sample {
            name: "es short UI",
            kind: SampleKind::ShortUi,
            expected: Some(Language::Spanish),
            route_language: None,
            ui_language: None,
            body_language: None,
            text: "Leer mas Suscribirse Volver Siguiente Buscar",
            ui_text: None,
            body_text: None,
        },
        Sample {
            name: "pt short UI",
            kind: SampleKind::ShortUi,
            expected: Some(Language::Portuguese),
            route_language: None,
            ui_language: None,
            body_language: None,
            text: "Leia mais Assinar Voltar Proximo Buscar",
            ui_text: None,
            body_text: None,
        },
        Sample {
            name: "id short UI",
            kind: SampleKind::ShortUi,
            expected: Some(Language::Indonesian),
            route_language: None,
            ui_language: None,
            body_language: None,
            text: "Baca selengkapnya Berlangganan Kembali Berikutnya Cari",
            ui_text: None,
            body_text: None,
        },
        Sample {
            name: "ru short UI",
            kind: SampleKind::ShortUi,
            expected: Some(Language::Russian),
            route_language: None,
            ui_language: None,
            body_language: None,
            text: "Читать дальше Подписаться Назад Далее Поиск",
            ui_text: None,
            body_text: None,
        },
        Sample {
            name: "de short UI",
            kind: SampleKind::ShortUi,
            expected: Some(Language::German),
            route_language: None,
            ui_language: None,
            body_language: None,
            text: "Mehr lesen Abonnieren Zuruck Weiter Suchen",
            ui_text: None,
            body_text: None,
        },
        Sample {
            name: "fr short UI",
            kind: SampleKind::ShortUi,
            expected: Some(Language::French),
            route_language: None,
            ui_language: None,
            body_language: None,
            text: "Lire la suite S'abonner Retour Suivant Rechercher",
            ui_text: None,
            body_text: None,
        },
        Sample {
            name: "en article",
            kind: SampleKind::Article,
            expected: Some(Language::English),
            route_language: None,
            ui_language: None,
            body_language: Some(Language::English),
            text: "Parents often notice bedtime problems only when the whole household is already tired. A reliable routine works because it reduces negotiation, gives children repeated cues, and makes the next step predictable. The exact schedule can change, but the sequence should stay calm and boring enough that the child does not need to keep testing it.",
            ui_text: None,
            body_text: None,
        },
        Sample {
            name: "es article",
            kind: SampleKind::Article,
            expected: Some(Language::Spanish),
            route_language: None,
            ui_language: None,
            body_language: Some(Language::Spanish),
            text: "Los padres suelen notar los problemas a la hora de dormir cuando toda la casa ya esta cansada. Una rutina fiable funciona porque reduce la negociacion, da senales repetidas a los ninos y hace que el siguiente paso sea predecible. El horario exacto puede cambiar, pero la secuencia debe mantenerse tranquila y constante.",
            ui_text: None,
            body_text: None,
        },
        Sample {
            name: "pt article",
            kind: SampleKind::Article,
            expected: Some(Language::Portuguese),
            route_language: None,
            ui_language: None,
            body_language: Some(Language::Portuguese),
            text: "Os pais muitas vezes percebem os problemas na hora de dormir quando toda a casa ja esta cansada. Uma rotina confiavel funciona porque reduz a negociacao, oferece sinais repetidos para a crianca e torna o proximo passo previsivel. O horario pode mudar, mas a sequencia deve continuar calma e consistente.",
            ui_text: None,
            body_text: None,
        },
        Sample {
            name: "id article",
            kind: SampleKind::Article,
            expected: Some(Language::Indonesian),
            route_language: None,
            ui_language: None,
            body_language: Some(Language::Indonesian),
            text: "Orang tua sering menyadari masalah tidur ketika seluruh rumah sudah lelah. Rutinitas yang konsisten membantu karena mengurangi perdebatan, memberi anak isyarat yang berulang, dan membuat langkah berikutnya mudah diprediksi. Jadwalnya bisa berubah, tetapi urutannya sebaiknya tetap tenang dan jelas.",
            ui_text: None,
            body_text: None,
        },
        Sample {
            name: "ru article",
            kind: SampleKind::Article,
            expected: Some(Language::Russian),
            route_language: None,
            ui_language: None,
            body_language: Some(Language::Russian),
            text: "Родители часто замечают проблемы со сном только тогда, когда вся семья уже устала. Надежный вечерний ритуал помогает, потому что уменьшает споры, дает ребенку повторяющиеся сигналы и делает следующий шаг предсказуемым. Расписание может меняться, но последовательность должна оставаться спокойной и понятной.",
            ui_text: None,
            body_text: None,
        },
        Sample {
            name: "de article",
            kind: SampleKind::Article,
            expected: Some(Language::German),
            route_language: None,
            ui_language: None,
            body_language: Some(Language::German),
            text: "Eltern bemerken Schlafprobleme oft erst, wenn der ganze Haushalt bereits erschopft ist. Eine verlassliche Routine hilft, weil sie Verhandlungen reduziert, dem Kind wiederkehrende Hinweise gibt und den nachsten Schritt vorhersehbar macht. Der genaue Zeitplan kann sich andern, aber die Reihenfolge sollte ruhig und konstant bleiben.",
            ui_text: None,
            body_text: None,
        },
        Sample {
            name: "fr article",
            kind: SampleKind::Article,
            expected: Some(Language::French),
            route_language: None,
            ui_language: None,
            body_language: Some(Language::French),
            text: "Les parents remarquent souvent les problemes de sommeil quand toute la maison est deja fatiguee. Une routine fiable fonctionne parce qu'elle reduit les negociations, donne a l'enfant des reperes repetes et rend l'etape suivante previsible. L'horaire exact peut changer, mais la sequence doit rester calme et constante.",
            ui_text: None,
            body_text: None,
        },
        Sample {
            name: "bad Spanish route and UI with English body",
            kind: SampleKind::MixedPage,
            expected: None,
            route_language: Some(Language::Spanish),
            ui_language: Some(Language::Spanish),
            body_language: Some(Language::English),
            text: "Ruta: /es/blog/como-dormir-mejor\nLeer mas Compartir Siguiente\nParents often notice bedtime problems only when the whole household is already tired. A reliable routine works because it reduces negotiation, gives children repeated cues, and makes the next step predictable. The exact schedule can change, but the sequence should stay calm and boring enough that the child does not need to keep testing it.",
            ui_text: Some("Leer mas Compartir Siguiente"),
            body_text: Some(
                "Parents often notice bedtime problems only when the whole household is already tired. A reliable routine works because it reduces negotiation, gives children repeated cues, and makes the next step predictable. The exact schedule can change, but the sequence should stay calm and boring enough that the child does not need to keep testing it.",
            ),
        },
        Sample {
            name: "good Spanish route and UI with Spanish body",
            kind: SampleKind::MixedPage,
            expected: Some(Language::Spanish),
            route_language: Some(Language::Spanish),
            ui_language: Some(Language::Spanish),
            body_language: Some(Language::Spanish),
            text: "Ruta: /es/blog/como-dormir-mejor\nLeer mas Compartir Siguiente\nLos padres suelen notar los problemas a la hora de dormir cuando toda la casa ya esta cansada. Una rutina fiable funciona porque reduce la negociacion, da senales repetidas a los ninos y hace que el siguiente paso sea predecible. El horario exacto puede cambiar, pero la secuencia debe mantenerse tranquila y constante.",
            ui_text: Some("Leer mas Compartir Siguiente"),
            body_text: Some(
                "Los padres suelen notar los problemas a la hora de dormir cuando toda la casa ya esta cansada. Una rutina fiable funciona porque reduce la negociacion, da senales repetidas a los ninos y hace que el siguiente paso sea predecible. El horario exacto puede cambiar, pero la secuencia debe mantenerse tranquila y constante.",
            ),
        },
        Sample {
            name: "ambiguous CTA",
            kind: SampleKind::Ambiguous,
            expected: None,
            route_language: None,
            ui_language: None,
            body_language: None,
            text: "Start",
            ui_text: None,
            body_text: None,
        },
    ]
}
