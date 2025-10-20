let clicked = false;

export function init_pickr(){
    var pickr = Pickr.create({
        el: '#btn',
        theme: 'monolith',
        default: "#3c3c3c",
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

    pickr.on("change", (color, source, istance) => {
        let elements = document.querySelector("#btn");
        elements.style.setProperty("background-color", "rgba(" + color.toRGBA()[0] + "," + color.toRGBA()[1] + "," + color.toRGBA()[2] + "," + color.toRGBA()[3] + ")")
    });

    
}


document.addEventListener('DOMContentLoaded', () => {
    const btn = document.querySelector('#btn');
    if (!btn) {return;}

    btn.addEventListener('click', () => {
        if (clicked) {
            clicked = false;
            pickr.hide();
        } else {
            clicked = true;
            pickr.show();
        }
    });
});