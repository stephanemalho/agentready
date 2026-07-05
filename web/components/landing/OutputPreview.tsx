import { CopyCommandButton } from "@/components/landing/CopyCommandButton";
import {
  JsonReportPreview,
  MarkdownReportPreview,
} from "@/components/landing/ReportPreviews";
import { Tabs, TabsContent, TabsList, TabsTrigger } from "@/components/ui/tabs";
import { installCommands } from "@/lib/landing";

export function OutputPreview() {
  return (
    <div className="space-y-4">
      <Tabs defaultValue="markdown">
        <TabsList>
          <TabsTrigger value="markdown" className="font-mono text-xs">
            --format markdown
          </TabsTrigger>
          <TabsTrigger value="json" className="font-mono text-xs">
            --format json
          </TabsTrigger>
        </TabsList>
        <TabsContent value="markdown" className="mt-4">
          <MarkdownReportPreview />
        </TabsContent>
        <TabsContent value="json" className="mt-4">
          <JsonReportPreview />
        </TabsContent>
      </Tabs>

      <div className="rounded-lg border bg-card/80 p-5">
        <p className="mb-3 font-mono text-xs font-semibold uppercase text-muted-foreground">
          Quick install
        </p>
        <div className="space-y-2">
          {installCommands.map((item) => (
            <div
              key={item.label}
              className="flex items-center gap-2 rounded-lg border bg-background/60 px-3 py-2"
            >
              <span
                aria-hidden="true"
                className="shrink-0 font-mono text-xs font-semibold text-primary"
              >
                $
              </span>
              <code className="min-w-0 flex-1 overflow-x-auto font-mono text-xs text-foreground">
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
