import { Skeleton } from "@/components/ui/skeleton";

export default function RepoScansLoading() {
  return (
    <div className="mx-auto w-full max-w-6xl space-y-6 px-5 py-10">
      <Skeleton className="h-8 w-64" />
      <Skeleton className="h-48" />
    </div>
  );
}
