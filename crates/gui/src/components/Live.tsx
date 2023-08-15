import '../css/Live.css'

import { CopyIcon, PlayIcon } from '../icon/icon'

interface Url {
    format: string
    url: string
}

interface LiveProps {
    live: string
    rid: string
    title: string
    anchor: string
    urls: Url[]
    img?: string
}

const Live = (props: LiveProps) => {
    return (
        <div class="live">
            <img class="live-img" src={props.img ?? '/src/assets/no_img.png'} />
            <div class="live-panel">
                <div class="live-title">{props.title}</div>
                <div class="live-control">
                    <button class="live-control-btn">
                        <CopyIcon size={15} />
                    </button>
                    <button class="live-control-btn">
                        <PlayIcon size={15} />
                    </button>
                </div>
            </div>
        </div>
    )
}

export default Live
