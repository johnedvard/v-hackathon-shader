vec3 white = vec3(1.0,1.0,1.0);
vec3 black = vec3(0.0,0.0,0.0);
vec3 orange = vec3(0.9, 0.6, 0.1);
vec3 lower_color_1 = vec3(1.0, 0.7, 0.5);
vec3 lower_color_2 = vec3(1.0, 1.0, 0.9);
float S = 75.0;
float PI = 3.14;


void render_image( out vec4 fragColor, in vec2 fragCoord )
{
  float y = fragCoord.y;
  float x = fragCoord.x;
  float t = y / (u_resolution.y - 1.0);
  float MID_Y = u_resolution.y * 0.5;

  float A = 15.0;
  float L = 75.0;
  float Hz = (2.0 * PI) / L;
  float curve_y = MID_Y + sin((x+u_time*S) * Hz) * A;
  float dist = curve_y - y;
  float alpha = (sign(dist) + 1.0) / 2.0;
  
  vec3 color_1 = white;
  vec3 color_2 = black;
  if(x >= u_resolution.x*0.3 && x <= u_resolution.x*0.7) {
    color_1 = orange;
    color_2 = orange;
  }
  
  
  vec3 upper_color = mix(color_1, color_2, t);
  vec3 lower_color = mix(lower_color_1, lower_color_2, t);
  
  vec3 color = mix(upper_color, lower_color, alpha);
  
  
  fragColor = vec4(color, 1.0);
}