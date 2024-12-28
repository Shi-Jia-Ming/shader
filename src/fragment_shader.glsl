#version 330 core

in vec3 ourColor;
out vec4 FragColor;

uniform sampler2D texture1;

void main() {
    vec3 shadowColor = vec3(1.0, 1.0, 1.0); // Shadow color
    float shadowIntensity = 0.5; // Shadow intensity

    // Calculate shadow effect
    vec3 color = mix(shadowColor, ourColor, shadowIntensity);

    FragColor = texture(texture1, gl_FragCoord.xy / 800.0) * vec4(color, 1.0);
}
