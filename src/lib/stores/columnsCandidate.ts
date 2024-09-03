import { persisted } from "svelte-persisted-store";

export type ColumnKeyCandidate = 'name' | 'location' | 'qualification' | 'hours' | 'mobility' | 'receivedAt' | 'notes' | 'startNote' | 'sentDocuments' | 'completedChecklist' | 'vaccinationStat' | 'certificationState' | 'personalDocumentation' | 'plannedChild';

export enum ColumnKeyCandidateEnum {
   // id = 'id',
    name = 'name',
    location = 'location',
    qualification = 'qualification',
    hours = 'hours',
    mobility = 'mobility',
    receivedAt = 'receivedAt',
    notes = 'notes',
    startNote = 'startNote',
    sentDocuments = 'sentDocuments',
    completedChecklist = 'completedChecklist',
    vaccinationStat = 'vaccinationStat',
    certificationState = 'certificationState',
    personalDocumentation = 'personalDocumentation',
    plannedChild = 'plannedChild'
}

export type ColumnLabelsCandidate = {
  [K in ColumnKeyCandidateEnum]: string;
};

export const defaultCandidateColumns: ColumnKeyCandidate[] = [
    ColumnKeyCandidateEnum.name, ColumnKeyCandidateEnum.location, ColumnKeyCandidateEnum.qualification, ColumnKeyCandidateEnum.hours, ColumnKeyCandidateEnum.mobility, ColumnKeyCandidateEnum.receivedAt, ColumnKeyCandidateEnum.notes, ColumnKeyCandidateEnum.startNote, ColumnKeyCandidateEnum.sentDocuments, ColumnKeyCandidateEnum.completedChecklist, ColumnKeyCandidateEnum.vaccinationStat, ColumnKeyCandidateEnum.certificationState, ColumnKeyCandidateEnum.personalDocumentation, ColumnKeyCandidateEnum.plannedChild
];

export const columnLabelsCandidate: ColumnLabelsCandidate = {
   // id: 'Match',
    name: 'Name',
    location: 'Ort',
    qualification: 'Qualif.',
    hours: 'Stundenumfang',
    mobility: 'Mobilität',
    receivedAt: 'Eingang',
    notes: 'Bemerkungen',
    startNote: 'Geplanter Start',
    sentDocuments: 'Unterlagen versendet',
    completedChecklist: 'Checkliste komplett',
    vaccinationStat: 'Massernschutz',
    certificationState: 'Führungszeugnis',
    personalDocumentation: 'Personalbogen',
    plannedChild: 'Geplantes Kind'
};

export const visibleCandidateColumns = persisted('candidateColumns', {
    defaultColumns: [...defaultCandidateColumns]
});

export const columnPositions: { [key in ColumnKeyCandidate]: number } = defaultCandidateColumns.reduce((acc, column, index) => {
  acc[column] = index;
  return acc;
}, {} as { [key in ColumnKeyCandidate]: number });
