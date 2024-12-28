#version 330 core

layout(location = 0) in vec3 aPos; // 输入顶点位置
layout(location = 1) in vec3 aColor; // 输入顶点颜色

out vec3 ourColor; // 输出顶点颜色

uniform vec3 offset; // 平移向量

void main() {
    // 平移操作：将输入顶点位置加上平移向量
    vec3 transformedPos = aPos;
    //  + vec3(0.85, 0.45, 0.0);

    // 设置顶点位置
    gl_Position = vec4(transformedPos, 1.0);
    ourColor = aColor;
}
