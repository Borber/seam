import "../css/SideItem.css"

import { A } from "@solidjs/router"
import { JSX } from "solid-js/jsx-runtime"

const SideItem = (props: {
    children: JSX.Element;
    path: string;
    bottom?: boolean;
    pathname: () => string;
}) => {
    return (
        <A href={props.path} class="side-link">
            <div
                class="side-item"
                classList={{
                    "side-item-bottom": props.bottom,
                    "side-item-selected": props.pathname() == props.path,
                }}>
                {props.children}
            </div>
        </A>
    )
}

export default SideItem
