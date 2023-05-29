import { useState, useEffect } from "react";
import { invoke } from "@tauri-apps/api/tauri";
import { Command } from "@tauri-apps/api/shell";
import brave from "./assets/brave.svg";
import chrome from "./assets/chrome.png";
import edge from "./assets/edge.svg";
import yandex from "./assets/yandex.png";

interface Logos {
  Brave: string;
  Edge: string;
  Chrome: string;
  Yandex: string;
}

const LOGOS: Logos = {
  Brave: brave,
  Edge: edge,
  Chrome: chrome,
  Yandex: yandex,
};

interface Browsers {
  Brave?: {
    id: number;
    name: string;
    data_dir: string;
  };
  Chrome?: {
    id: number;
    name: string;
    data_dir: string;
  };
  Edge?: {
    id: number;
    name: string;
    data_dir: string;
  };
  Yandex?: {
    id: number;
    name: string;
    data_dir: string;
  };
}

function App() {
  const [browsers, setBrowsers] = useState<Browsers | null>(null);
  const [selected, setSelected] = useState<string>(
    () => localStorage.getItem("selectedBrowser") || ""
  );
  const [profiles, setProfiles] = useState([]);
  const [selectedIndex, setSelectedIndex] = useState(0);

  useEffect(() => {
    (async () => {
      const browsers: Browsers | null = await invoke("get_browsers");
      setBrowsers(browsers);
      if (selected === "") {
        /* @ts-ignore */
        setSelected(Object.keys(browsers)[0]);
      }
    })();
  }, []);

  useEffect(() => {
    (async () => {
      const profiles: any = await invoke("get_profiles", {
        name: selected,
      });
      setProfiles(profiles);
    })();
  }, [selected]);

  const onChangeInput = (value: string) => {
    setSelected(value);
    localStorage.setItem("selectedBrowser", value);
  };

  const onChangeProfile = async (item: any, index: number) => {
    setSelectedIndex(index);
    // @ts-ignore
    console.log(browsers[selected].name);
    try {
      await new Command("osascript", [
        "-e",
        'tell application "System Events"',
        "-e",
        /* @ts-ignore */
        `tell process "${browsers[selected].name}"`,
        "-e",
        `click menu item "${item.name}" of menu 1 of menu bar item "个人资料" of menu bar 1`,
        "-e",
        "end tell",
        "-e",
        "end tell",
      ]).execute();
    } catch (e) {
      console.log(e);
    }
  };

  if (browsers === null) {
    return (
      <div className="p-4 flex items-center justify-center h-screen">
        <h1 className="dark:text-gray-200 text-3xl">未找到浏览器数据！！！</h1>
      </div>
    );
  }

  return (
    <>
      <div className="p-2 dark:text-gray-200">
        <div className="flex">
          {Object.entries(browsers)
            .sort(([, ov], [, nv]) => (ov.id > nv.id ? 1 : -1))
            .map(([name, item]) => (
              <label
                key={item.id}
                className={`flex items-center justify-center px-2 py-0.5 select-none ${
                  selected === name
                    ? "dark:bg-gray-500/50 bg-white/50 rounded"
                    : ""
                }`}
              >
                <input
                  type="radio"
                  value={name}
                  name="ibrowser"
                  className="hidden"
                  checked={selected === name}
                  onChange={(e) => onChangeInput(e.target.value)}
                />
                {/* @ts-ignore */}
                <img src={LOGOS[name]} className="w-4 h-4 mr-1" />
                <span>{name}</span>
              </label>
            ))}
          <button
            data-tauri-drag-region
            className={`flex items-center justify-center px-2 py-0.5 select-none dark:hover:bg-gray-500/50 rounded ml-2`}
          >
            Move
          </button>
        </div>
        <div className="space-x-2 space-y-1">
          {/* @ts-ignore */}
          {profiles.map((item, index) => (
            <button
              key={index}
              className={`rounded px-2 whitespace-nowrap ${
                index === selectedIndex
                  ? "bg-red-500"
                  : "bg-white/50 dark:bg-gray-700/50"
              }`}
              onClick={() => onChangeProfile(item, index)}
            >
                {/* @ts-ignore */}
              {item.name}
            </button>
          ))}
        </div>
      </div>
    </>
  );
}

export default App;
