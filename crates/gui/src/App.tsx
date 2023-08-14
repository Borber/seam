import './App.css'
import './css/TopBar.css'

import { useRoutes } from '@solidjs/router'
import { lazy, onMount } from 'solid-js'
import { Toaster } from 'solid-toast'

import Control from './components/Control'
import SideBar from './components/SideBar'
import TopBar from './components/TopBar'

// interface Node {
//     rid: string
//     title: string
//     urls: Url[]
// }

// interface Url {
//     format: string
//     url: string
// }

const App = () => {
    onMount(async () => {
        // 生产环境, 全局取消右键菜单;
        if (!import.meta.env.DEV) {
            document.oncontextmenu = (event) => {
                event.preventDefault()
            }
        }

        // 避免窗口闪烁, 等待500ms再显示窗口
        // 这个该死的bug什么时候才能修
        setTimeout(() => {
            setupWindow()
        }, 500)
    })

    const setupWindow = async () => {
        const appWindow = (await import('@tauri-apps/api/window')).appWindow
        appWindow.show()
    }

    const routes = [
        { path: '/', component: lazy(() => import('./pages/Home')) },
        { path: '/good', component: lazy(() => import('./pages/Good')) },
        { path: '/setting', component: lazy(() => import('./pages/Setting')) },
    ]

    const Routes = useRoutes(routes)

    return (
        <>
            <Control maximize={false} />
            <TopBar />
            <div class="container  not-draggable">
                <SideBar />
                <div class="content">
                    <Routes />
                </div>
            </div>
            <Toaster
                position="bottom-center"
                gutter={8}
                toastOptions={{
                    className: '',
                    duration: 5000,
                    style: {
                        background: '#0f0f0fc9',
                        color: '#fff',
                    },
                }}
            />
        </>
    )
}

export default App
