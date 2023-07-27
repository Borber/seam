import "./App.css";

import { invoke } from "@tauri-apps/api/tauri";
import { createSignal, For, onMount } from "solid-js";

import logo from "./assets/logo.png";
import Bar from "./components/Bar";
import SilderItem, { SilderItemProps } from "./components/SilderItem";

const App = () => {
    const [repos] = createSignal<SilderItemProps[]>([]);

    onMount(async () => {
        // 全局取消右键菜单
        // document.oncontextmenu = (event) => {
        //   event.preventDefault();
        // };

        // 避免窗口闪烁, 等待500ms再显示窗口
        // 这个该死的bug什么时候才能修
        setTimeout(() => {
            setupWindow();
        }, 500);
    });

    const setupWindow = async () => {
        const appWindow = (await import("@tauri-apps/api/window")).appWindow;
        appWindow.show();
    };

    invoke<string>("greet");

    return (
        <>
            <div class="container  not-draggable">
                <div class="sider">
                    <For each={repos()}>
                        {(item) => (
                            <SilderItem name={item.name} path={item.path} />
                        )}
                    </For>
                </div>
                <div data-tauri-drag-region class="loader">
                    <Bar maximize={false} />

                    <div>
                        <img src={logo} alt="Logo" width={"100px"} />
                        <div class="name">Seam</div>
                        <div class="version">0.1.0</div>
                    </div>
                </div>
            </div>
        </>
    );
};

export default App;
