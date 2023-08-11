interface IconProps {
    size: number;
}

const CopyIcon = (props: IconProps) => {
    return (
        <svg
            class="icon"
            viewBox="0 0 1024 1024"
            version="1.1"
            xmlns="http://www.w3.org/2000/svg"
            p-id="1472"
            data-darkreader-inline-fill=""
            width={props.size}
            height={props.size}>
            <path
                d="M896 213.333333v512h-85.333333V213.333333H298.666667V128h597.333333v85.333333z m-128 128v554.666667H128V256h640v85.333333z m-85.333333 0H213.333333v469.333334h469.333334V341.333333z"
                fill="#e6e6e6"
                p-id="1473"
                data-darkreader-inline-fill=""
                style={{ "--darkreader-inline-fill": "#0a1620" }}
            />
        </svg>
    );
};

const PlayIcon = (props: IconProps) => {
    return (
        <svg
            class="icon"
            viewBox="0 0 1024 1024"
            version="1.1"
            xmlns="http://www.w3.org/2000/svg"
            p-id="2732"
            width={props.size}
            height={props.size}>
            <path
                d="M170.666667 896l0-768 640 384zM644.650667 512l-388.650667-233.344 0 466.688z"
                p-id="2733"
                fill="#e6e6e6"
            />
        </svg>
    );
};

export { CopyIcon, PlayIcon };
