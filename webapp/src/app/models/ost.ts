export enum OstType {
    Opening = "Opening",
    Ending = "Ending",
}

export interface Ost {
    proxer_id: number;
    ost_type: OstType;
    number: number;
    name: string | null,
    artist: string | null,
    video_url: string | null,
}