import { useEffect, useState } from "react";
import { invoke } from "@tauri-apps/api/core";
import "./DistroList.css";

export default function DistroList() {
  const [distros, setDistros] = useState<string[]>([]);

  useEffect(() => {
    invoke<string[]>("get_distros").then(setDistros).catch(console.error);
  }, []);

  return (
    <div className="distroList">
      {distros.map((distro) => (
        <DistroCard key={distro} name={distro} />
      ))}
    </div>
  );
}

type DistroCardProps = {
  name: string;
};

function DistroCard({ name }: DistroCardProps) {
  return (
    <div className="distroContainer">
      <h2 className="">{name}</h2>
      <div className="funcBtn">
        <button className="launchBtn">Launch</button>
        <button className="settingsBtn">Settings</button>
      </div>
    </div>
  );
}
