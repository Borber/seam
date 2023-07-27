import "../css/SilderItem.css";

export interface SilderItemProps {
    name: string;
    path: string;
}

const More = () => {
    return (
        <svg
            xmlns="http://www.w3.org/2000/svg"
            width="24"
            height="24"
            viewBox="0 0 24 24"
            fill="none"
            stroke="currentColor"
            stroke-width="2"
            stroke-linecap="round"
            stroke-linejoin="round"
            class="sider-item-svg-menu-icon">
            <circle cx="12" cy="12" r="1" />
            <circle cx="12" cy="5" r="1" />
            <circle cx="12" cy="19" r="1" />
        </svg>
    );
};

const SilderItem = (props: SilderItemProps) => {
    return (
        <div class="sider-item">
            <div class="sider-item-info">
                <div class="sider-item-name">{props.name}</div>
                <div class="sider-item-path">{props.path}</div>
            </div>
            <div class="sider-item-menu">
                <More />
            </div>
        </div>
    );
};

export default SilderItem;
