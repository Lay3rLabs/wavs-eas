export interface AttestationData {
  uid: string;
  schemaId: string;
  refUID: string;
  data: string;
}

export interface ContainmentResult {
  attestationId: string;
  location: any;
  isContainedInPolygon: boolean;
}