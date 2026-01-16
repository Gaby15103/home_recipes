export interface Tag {
    id: string;
    name: string;
}

export type InputTag =
    | { type: 'existing'; id: string }
    | { type: 'new'; name: string }