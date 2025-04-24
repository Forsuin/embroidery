export type Pattern = {
    id: number;
    name: string;
    pattern_num?: number;
    thread_count?: number;
};

export type Tag = {
    id?: number;
    name: string;
};

export type PatternTags = {
    pattern: Pattern;
    tags: Tag[];
};