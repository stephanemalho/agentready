import { CopyCommandButton } from "@/components/landing/CopyCommandButton";
import {
  JsonReportPreview,
  MarkdownReportPreview,
} from "@/components/landing/ReportPreviews";
import { Tabs, TabsContent, TabsList, TabsTrigger } from "@/components/ui/tabs";
import { installCommands } from "@/lib/landing";

export function OutputPreview() {
  return (
    <div className="min-w-0 space-y-4">
      <Tabs defaultValue="markdown" className="min-w-0">
        <TabsList className="max-w-full overflow-x-auto">
          <TabsTrigger value="markdown" className="font-mono text-xs">
            --format markdown
          </TabsTrigger>
          <TabsTrigger value="json" className="font-mono text-xs">
            --format json
          </TabsTrigger>
        </TabsList>
        <TabsContent value="markdown" className="mt-4 min-w-0">
          <MarkdownReportPreview />
        </TabsContent>
        <TabsContent value="json" className="mt-4 min-w-0">
          <JsonReportPreview />
        </TabsContent>
      </Tabs>

      <div className="bg-card/80 min-w-0 rounded-lg border p-5">
        <p className="text-muted-foreground mb-3 font-mono text-xs font-semibold uppercase">
          Quick install
        </p>
        <div className="min-w-0 space-y-2">
          {installCommands.map((item) => (
            <div
              key={item.label}
              className="bg-background/60 flex min-w-0 items-center gap-2 rounded-lg border px-3 py-2"
            >
              <span
                aria-hidden="true"
                className="text-primary shrink-0 font-mono text-xs font-semibold"
              >
                $
              </span>
              <code className="text-foreground min-w-0 flex-1 overflow-x-auto font-mono text-xs whitespace-nowrap">
                {item.command}
              </code>
              <CopyCommandButton command={item.command} />
            </div>
          ))}
        </div>
      </div>
    </div>
  );
}
