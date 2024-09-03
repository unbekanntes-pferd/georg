import { persisted } from "svelte-persisted-store";


export type ColumnKeySchoolAssistant = 'lastName' | 'firstName' | 'birthDate' | 'assignedChild' | 'telNumber' | 'mobileNumber' | 'email' | 'address' | 'zipCode' | 'city' | 'level' | 'approved' | 'info' | 'certifications' | 'title' | 'children' | 'assitantSince';

export enum ColumnKeySchoolAssistantEnum {
    lastName = 'lastName',
    firstName = 'firstName',
    birthDate = 'birthDate',
    assignedChild = 'assignedChild',
    telNumber = 'telNumber',
    mobileNumber = 'mobileNumber',
    email = 'email',
    address = 'address',
    zipCode = 'zipCode',
    city = 'city',
    level = 'level',
    approved = 'approved',
    info = 'info',
    certifications = 'certifications',
    title = 'title',
    children = 'children',
    assitantSince = 'assitantSince'
}

export type ColumnLabelsSchoolAssistant = {
    [K in ColumnKeySchoolAssistantEnum]: string;
  };

export const defaultSchoolAssistantColumns: ColumnKeySchoolAssistant[] = [
    ColumnKeySchoolAssistantEnum.lastName, ColumnKeySchoolAssistantEnum.firstName, ColumnKeySchoolAssistantEnum.birthDate, ColumnKeySchoolAssistantEnum.assignedChild, ColumnKeySchoolAssistantEnum.telNumber, ColumnKeySchoolAssistantEnum.mobileNumber, ColumnKeySchoolAssistantEnum.email, ColumnKeySchoolAssistantEnum.address, ColumnKeySchoolAssistantEnum.zipCode, ColumnKeySchoolAssistantEnum.city, ColumnKeySchoolAssistantEnum.level, ColumnKeySchoolAssistantEnum.approved, ColumnKeySchoolAssistantEnum.info, ColumnKeySchoolAssistantEnum.certifications, ColumnKeySchoolAssistantEnum.title, ColumnKeySchoolAssistantEnum.children, ColumnKeySchoolAssistantEnum.assitantSince
];

export const columnLabelsSchoolAssistant: ColumnLabelsSchoolAssistant = {
    lastName: 'Nachname',
    firstName: 'Vorname',
    birthDate: 'Geb. Datum',
    assignedChild: 'Begl. Kind',
    telNumber: 'Telefon',
    mobileNumber: 'Mobil',
    email: 'Email',
    address: 'Straße',
    zipCode: 'PLZ',
    city: 'Wohnort',
    level: 'Eigr.',
    approved: 'Genehm',
    info: 'Info',
    certifications: 'Abschlusszeugnisse',
    title: 'Berufsbezeichnung / Ausbildung',
    children: 'Kinder',
    assitantSince: 'Zugehörigkeit'
};

export const visibleSchoolAssistantColumns = persisted('schoolAssistantColumns', {
    defaultColumns: [...defaultSchoolAssistantColumns]
});

export const columnPositions: { [key in ColumnKeySchoolAssistant]: number } = defaultSchoolAssistantColumns.reduce((acc, column, index) => {
  acc[column] = index;
  return acc;
}, {} as { [key in ColumnKeySchoolAssistant]: number });