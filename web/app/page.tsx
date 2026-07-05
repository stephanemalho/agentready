import { FeaturesPreview } from "@/components/landing/FeaturesPreview";
import { HarnessMatrix } from "@/components/landing/HarnessMatrix";
import { HomeHero } from "@/components/landing/HomeHero";
import { InstallCallout } from "@/components/landing/InstallCallout";

export default function HomePage() {
  return (
    <>
      <HomeHero />
      <FeaturesPreview />
      <HarnessMatrix />
      <InstallCallout />
    </>
  );
}
