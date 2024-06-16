export interface Candidate {
	id: string;
	name: string;
	location: string;
	qualification: string | null;
	hours: string | null;
	mobility: string | null;
	receivedAt: Date;
	notes: string | null;
	startNote: string | null;
	sentDocuments: string | null;
	completedChecklist: string | null;
	vaccinationStat: string | null;
	certificationState: string | null;
	personalDocumentation: string | null;
	plannedChild: string | null;
}

export interface ChildCareRequest {
	id: string;
	institution: string;
	location: string;
	grade: string;
	hours: string;
	diagnosis: string | null;
	contact: string;
	receivedAt: Date;
	notes: string | null;
}

export interface SchoolAssistant {
	id: string;
    lastName: string;
    firstName: string;
    birthDate: Date;
    accompanyingChild: string;
    phone?: string;
    mobile?: string;
    email: string;
    streetHouseNumber: string;
    postalCode: string;
    city: string;
    level: string;
    approved: string;
    info?: string;
    certificates: string;
    professionalQualification: string;
    children: string;
    membershipDate: Date;
}

export interface AccompaniedChild {
	id: string;
	name: string;
	qualification: string;
	approvalUntil: Date;
	fundingAgency: string;
	contactPerson: string;
	contactPhone: string;
	contactEmail: string;
	notes: string;
}

export interface ResponseGetCandidates {
	candidates: Candidate[];
	childCareRequests: ChildCareRequest[];
}

export interface ResponseGetCandidateMatches {
	candidate: ChildCareRequest,
	distance: number
}

export interface ResponseGetChildcareMatches {
	candidate: Candidate,
	distance: number
}

export interface ResponseGetSchoolAssistants {
	assistants: SchoolAssistant[];
}

export interface ResponseGetAssistantsAndAccompaniedChildren {
	assistants: SchoolAssistant[];
	children: AccompaniedChild[];
}

export interface ResponseGetSchoolAssistantMatches {
	assistant: SchoolAssistant,
	distance: number
}

export interface ResponseGetAccompaniedChildMatches {
	assistant: SchoolAssistant,
	distance: number
}

export function getFullName(assistant: SchoolAssistant): string {
	return `${assistant.firstName} ${assistant.lastName}`;
}

export function getFullAddress(assistant: SchoolAssistant): string {
	return `${assistant.streetHouseNumber}, ${assistant.postalCode} ${assistant.city}`;
}

