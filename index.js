/****************************************************************************************************************************************
# * index.js                                                                                                                          *
#  **************************************************************************************************************************************
#  *                                                                                                                                    *
#  * @License Starts                                                                                                                    *
#  *                                                                                                                                    *
#  * Copyright © 2015 - present. MongoExpUser.  All Rights Reserved.                                                                    *
#  *                                                                                                                                    *
#  * License: MIT - https://github.com/MongoExpUser/Debian-Based-NAPI-Rust-Addons/blob/main/LICENSE                                     *
#  *                                                                                                                                    *
#  * @License Ends                                                                                                                      *
#  **************************************************************************************************************************************
# *                                                                                                                                     *
# *  Project: Rust Container Project for NAPI-Rust                                                                                      *
# *                                                                                                                                     *
# *  This index.js file creates a simple class to test:                                                                                 *
# *                                                                                                                                     *                                                                                                              *
# *   1)  Invocation of a NAPI-Rust Addon module from within Node.js                                                                    *
# *                                                                                                                                     *
# *                                                                                                                                     *
# ***************************************************************************************************************************************/



class OilApiGravity
{
    constructor()
    {
      return null;
    }
    
    oilApiGravity(specificGravity)
    {
      const { api } = require("./api.node");
      return api(specificGravity);
    }
}


(async function test()
{
  const oag = new  OilApiGravity();
  oag.oilApiGravity(1.02);
  // value should be 8
 
})();
