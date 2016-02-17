//
//  Shader.fsh
//  trafi-inspector
//
//  Created by Nerijus on 17/02/16.
//  Copyright © 2016 Trafi. All rights reserved.
//

varying lowp vec4 colorVarying;

void main()
{
    gl_FragColor = colorVarying;
}
