import { fakerDE as faker } from '@faker-js/faker';

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

export interface ResponseGetAssistantsAndAccompaniedChildren {
	assistants: SchoolAssistant[];
	children: AccompaniedChild[];
}

export interface ResponseGetSchoolAssistantMatches {
	child: AccompaniedChild,
	distance: number
}

export interface ResponseGetAccompaniedChildMatches {
	assistant: SchoolAssistant,
	distance: number
}



function getRandomElement<T>(items: T[]): T {
	return items[Math.floor(Math.random() * items.length)];
}

export function randomizeCandidate(): Candidate {
	return {
		id: faker.string.uuid(),
		name: faker.person.fullName(),
		location: faker.location.city(),
		qualification: getRandomElement(['Bachelor', 'Master', 'PhD', 'Diploma']),
		hours: faker.datatype.boolean() ? getRandomElement(['Full-time', 'Part-time']) : null,
		mobility: faker.datatype.boolean() ? faker.vehicle.type() : null,
		receivedAt: faker.date.past(),
		notes: faker.datatype.boolean() ? faker.lorem.sentence() : null,
		startNote: faker.datatype.boolean() ? faker.lorem.sentence() : null,
		sentDocuments: faker.datatype.boolean() ? 'Yes' : null,
		completedChecklist: faker.datatype.boolean() ? 'Yes' : null,
		vaccinationStat: faker.datatype.boolean()
			? getRandomElement(['Vaccinated', 'Not Vaccinated'])
			: null,
		certificationState: faker.datatype.boolean()
			? getRandomElement(['Certified', 'Not Certified'])
			: null,
		personalDocumentation: faker.datatype.boolean() ? 'Available' : null,
		plannedChild: faker.datatype.boolean() ? 'Yes' : null
	};
}

export function generateCandidates(count: number): Candidate[] {
	const candidates: Candidate[] = [];
	for (let i = 0; i < count; i++) {
		candidates.push(randomizeCandidate());
	}
	return candidates;
}

export function randomizeChildCareRequest(): ChildCareRequest {
	return {
		id: faker.string.uuid(),
		institution: faker.company.name(),
		location: faker.location.city(),
		grade: faker.number.int({ min: 0, max: 5 }).toString(),
		hours: getRandomElement(['Full-time', 'Part-time']),
		diagnosis: faker.datatype.boolean() ? faker.lorem.sentence() : null,
		contact: faker.phone.number(),
		receivedAt: faker.date.past(),
		notes: faker.datatype.boolean() ? faker.lorem.sentence() : null
	};
}

export function generateChildCareRequests(count: number): ChildCareRequest[] {
	const requests: ChildCareRequest[] = [];
	for (let i = 0; i < count; i++) {
		requests.push(randomizeChildCareRequest());
	}
	return requests;
}

export function randomizeSchoolAssistant(): SchoolAssistant {
	return {
		id: faker.string.uuid(),
		lastName: faker.person.lastName(),
		firstName: faker.person.firstName(),
		birthDate: faker.date.past(),
		accompanyingChild: faker.person.fullName(),
		phone: faker.phone.number(),
		mobile: faker.phone.number(),
		email: faker.internet.email(),
		streetHouseNumber: faker.location.streetAddress(),
		postalCode: faker.location.zipCode(),
		city: faker.location.city(),
		level: getRandomElement(['Primary', 'Secondary', 'High']),
		approved: getRandomElement(['Yes', 'No']),
		info: faker.lorem.sentence(),
		certificates: faker.lorem.sentence(),
		professionalQualification: faker.lorem.sentence(),
		children: faker.lorem.sentence(),
		membershipDate: faker.date.past()
	};
}

export function generateSchoolAssistants(count: number): SchoolAssistant[] {
	const assistants: SchoolAssistant[] = [];
	for (let i = 0; i < count; i++) {
		assistants.push(randomizeSchoolAssistant());
	}
	return assistants;
}

export function randomizeAccompaniedChild(): AccompaniedChild {
	return {
		id: faker.string.uuid(),
		name: faker.person.fullName(),
		qualification: getRandomElement(['Primary', 'Secondary', 'High']),
		approvalUntil: faker.date.future(),
		fundingAgency: faker.company.name(),
		contactPerson: faker.person.fullName(),
		contactPhone: faker.phone.number(),
		contactEmail: faker.internet.email(),
		notes: faker.lorem.sentence()
	};
}

export function generateAccompaniedChildren(count: number): AccompaniedChild[] {
	const children: AccompaniedChild[] = [];
	for (let i = 0; i < count; i++) {
		children.push(randomizeAccompaniedChild());
	}
	return children;
}