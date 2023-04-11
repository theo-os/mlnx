let
  textureSampler = builtins.uniform "sampler2D" 0;
in
{
  type = "fragment";

  color = {
    location = 0;
    value = builtins.texture textureSampler (builtins.input "vec2" "texCoord");
  };
}
