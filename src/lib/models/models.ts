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

export interface Assistant {
	id: string;
    lastName: string;
    firstName: string;
    birthDate: Date;
    assignedChild: string;
    telNumber?: string;
    mobileNumber?: string;
    email: string;
    address: string;
    zipCode: number;
    city: string;
    level: string;
    approved: string;
    info?: string;
    certifications: string;
    title: string;
    children: string;
    assitantSince: string;
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
	assistants: Assistant[];
}

export interface ResponseGetAssistantsAndAccompaniedChildren {
	assistants: Assistant[];
	children: AccompaniedChild[];
}

export interface ResponseGetSchoolAssistantMatches {
	assistant: Assistant,
	distance: number
}

export interface ResponseGetAccompaniedChildMatches {
	assistant: Assistant,
	distance: number
}

export function getFullName(assistant: Assistant): string {
	return `${assistant.firstName} ${assistant.lastName}`;
}

export function getFullAddress(assistant: Assistant): string {
	return `${assistant.address}, ${assistant.zipCode} ${assistant.city}`;
}

