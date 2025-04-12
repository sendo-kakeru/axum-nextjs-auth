export type PageProps<P extends string> = {
  params: Promise<{ [slug in P]: string }>;
  searchParams: Promise<{ [key: string]: string | string[] | undefined }>;
};
