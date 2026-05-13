use sqlx::query_as as qa;

pub fn documented_query() {
    let _row = qa!(ManualInput, "select 1");
}

pub fn weak_query() {
    let _row = qa!(ManualInput, "select 1");
}
