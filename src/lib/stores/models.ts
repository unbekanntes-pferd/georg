import type { Writable } from "svelte/store";
import type { ColumnKeyCandidate, } from "./columnsCandidate";
import type { ColumnKeyChildCare } from "./columnsChildcare";
import type { ColumnKeySchoolAssistant } from "./columnsSchoolAssistant";

export interface Persisted<T> extends Writable<T> {
    reset: () => void;
}

// Define a generic type T which will be constrained to one of the three column key types
export type ColumnKey = ColumnKeyCandidate | ColumnKeyChildCare | ColumnKeySchoolAssistant;

export enum ColumnType {
    Candidate = 'Candidate',
    ChildCare = 'ChildCare',
    SchoolAssistant = 'SchoolAssistant'
}