import '../css/SideBar.css'

import { useLocation } from '@solidjs/router'
import { createMemo } from 'solid-js'

import { GoodIcon, HomeIcon, SettingIcon } from '../icon/icon'
import SideItem from './SideItem'

const SideBar = () => {
    const pathname = createMemo(() => {
        return useLocation().pathname
    })
    return (
        <div data-tauri-drag-region class="side-bar">
            <SideItem path="/" pathname={pathname}>
                <HomeIcon size={30} />
            </SideItem>
            <SideItem path="/good" pathname={pathname}>
                <GoodIcon size={30} />
            </SideItem>
            <SideItem path="/setting" pathname={pathname} bottom={true}>
                <SettingIcon size={30} />
            </SideItem>
        </div>
    )
}

export default SideBar
