import type { SpotlightCourse } from "@/types/content";

export const SITE_NAME = "Steady Home";
export const SITE_TAGLINE = "Calm systems for busy families";

export const SPOTLIGHT_COURSES: SpotlightCourse[] = [
  {
    slug: "bedtime-reset",
    title: "Bedtime Reset",
    summary: "A five-night routine for reducing bedtime friction without power struggles.",
    audience: "Parents of kids who delay, negotiate, and pop back out of bed.",
    ctaLabel: "See the bedtime reset",
  },
  {
    slug: "weekly-planning",
    title: "Weekly Planning",
    summary: "Build a family plan that survives school nights, sports, and surprise errands.",
    audience: "Families juggling school logistics, care work, and household admin.",
    ctaLabel: "Open the weekly planning guide",
  },
];

export const COMMUNITY_STATS = [
  { label: "families coached", value: "4,200+" },
  { label: "playbooks delivered", value: "18" },
  { label: "average setup time", value: "12 min" },
];

export const TRUST_SIGNALS = [
  "Scripts for school-night transitions",
  "Planning templates that survive bad weeks",
  "Low-shame routines for inconsistent households",
];
