export enum OstType {
    Opening,
    Ending,
}

export interface Ost {
    proxer_id: number;
    ost_type: OstType;
    number: number;
    name?: string,
    artist?: string,
    video_url?: string,
}