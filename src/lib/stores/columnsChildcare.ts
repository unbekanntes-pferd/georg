import { persisted } from "svelte-persisted-store";

export type ColumnKeyChildCare = 'id' | 'institution' | 'location' | 'grade' | 'hours' | 'diagnosis' | 'contact' | 'receivedAt' | 'notes';

export enum ColumnKeyChildCareEnum {
    id = 'id',
    institution = 'institution',
    location = 'location',
    grade = 'grade',
    hours = 'hours',
    diagnosis = 'diagnosis',
    contact = 'contact',
    receivedAt = 'receivedAt',
    notes = 'notes'
}

export type ColumnLabelsChildCare = {
  [K in ColumnKeyChildCareEnum]: string;
};

export const defaultChildCareColumns: ColumnKeyChildCare[] = [
    ColumnKeyChildCareEnum.id, ColumnKeyChildCareEnum.institution, ColumnKeyChildCareEnum.location, ColumnKeyChildCareEnum.grade, ColumnKeyChildCareEnum.hours, ColumnKeyChildCareEnum.diagnosis, ColumnKeyChildCareEnum.contact, ColumnKeyChildCareEnum.receivedAt, ColumnKeyChildCareEnum.notes
];

export const columnLabelsChildCare: ColumnLabelsChildCare = {
    id: 'Match',
    institution: 'Einrichtung',
    location: 'Ort',
    grade: 'Klasse',
    hours: 'Stunden',
    diagnosis: 'Diagnose',
    contact: 'Ansprechpartner*in',
    receivedAt: 'Datum',
    notes: 'Bemerkung'
};

export const visibleChildCareColumns = persisted('childCareColumns', {
  defaultColumns: [...defaultChildCareColumns]
});

export const columnPositions: { [key in ColumnKeyChildCare]: number } = defaultChildCareColumns.reduce((acc, column, index) => {
  acc[column] = index;
  return acc;
}, {} as { [key in ColumnKeyChildCare]: number });
