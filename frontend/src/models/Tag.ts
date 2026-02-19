export interface Tag {
    id: string;
    name: string;
}

export type InputTag =
    | { type: 'Existing'; id: string }
    | { type: 'New'; name: string }
