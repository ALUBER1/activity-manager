let clicked = false;

export function init_pickr(id, def){
    let pickr = Pickr.create({
        el: id,
        theme: 'monolith',
        default: def,
        useAsButton: true,
        components: {
            opacity: true,
            hue: true,
            interaction: {
                hex: true,
                rgba: true,
                input: true
            }
        }
    });

    pickr.on("change", (color, source, instance) => {
        const el = instance.options.el;
        const rgba = color.toRGBA();
        el.style.setProperty(
            "background-color",
            `rgba(${rgba[0]},${rgba[1]},${rgba[2]},${rgba[3]})`
        );
    });

    document.addEventListener('DOMContentLoaded', () => {
        const btn = document.querySelectorAll(".pickr");
        if (!btn) {return;}

        for (el of btn) {
            el.addEventListener('click', () => {
                if (clicked) {
                    clicked = false;
                    pickr.hide();
                } else {
                    clicked = true;
                    pickr.show();
                }
            });
        }
    });
}


