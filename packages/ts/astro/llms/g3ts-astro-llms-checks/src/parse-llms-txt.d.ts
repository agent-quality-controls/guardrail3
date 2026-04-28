declare module "parse-llms-txt" {
  export interface LlmsTxtFileEntry {
    name: string;
    url: string;
    notes?: string;
  }

  export interface LlmsTxtSection {
    name: string;
    files: LlmsTxtFileEntry[];
  }

  export interface LlmsTxtFile {
    title: string;
    description?: string;
    details?: string;
    sections: LlmsTxtSection[];
  }

  export function parseLlmsTxt(markdown: string): LlmsTxtFile;
}
