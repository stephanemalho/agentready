import type { Metadata } from "next";
import { Geist, Geist_Mono } from "next/font/google";

import { BackendStatusProvider } from "@/components/BackendStatusProvider";
import { SiteHeader } from "@/components/landing/SiteHeader";
import { ThemeProvider } from "@/components/ThemeProvider";
import "./globals.css";

const geistSans = Geist({
  variable: "--font-geist-sans",
  subsets: ["latin"],
});

const geistMono = Geist_Mono({
  variable: "--font-geist-mono",
  subsets: ["latin"],
});

export const metadata: Metadata = {
  title: "AgentReady",
  description:
    "Check whether a repository is ready for coding-agent harnesses such as Codex, Claude Code, and Gemini CLI.",
  icons: {
    icon: [{ url: "/icon.svg", type: "image/svg+xml" }],
    shortcut: ["/icon.svg"],
  },
};

export default function RootLayout({
  children,
}: Readonly<{
  children: React.ReactNode;
}>) {
  return (
    <html
      lang="en"
      className={`${geistSans.variable} ${geistMono.variable} h-full antialiased`}
      suppressHydrationWarning
    >
      <body className="flex min-h-full flex-col">
        <ThemeProvider>
          <BackendStatusProvider>
            <SiteHeader />
            <main className="w-full flex-1">{children}</main>
          </BackendStatusProvider>
        </ThemeProvider>
      </body>
    </html>
  );
}
