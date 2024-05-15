import { fakerDE as faker } from '@faker-js/faker';

interface Candidate {
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

export interface ResponseGetCandidates {
	candidates: Candidate[];
	childCareRequests: ChildCareRequest[];
}

function getRandomElement<T>(items: T[]): T {
	return items[Math.floor(Math.random() * items.length)];
}

export function randomizeCandidate(): Candidate {
	return {
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
