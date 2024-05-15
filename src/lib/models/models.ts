interface Candidate {
	name: string;
	location: string;
	qualification: string;
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

interface ChildCareRequest {
	institution: string;
	location: string;
	grade: string;
	hours: string;
	diagnosis: string | null;
	contact: string;
	receivedAt: Date;
	notes: string | null;
}

interface ResponseGetCandidates {
	data: Candidate[];
}
