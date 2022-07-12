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
# *   1)  Invocation of a NAPI-Rust Addon modules from within Node.js                                                                    *
# *                                                                                                                                     *
# *                                                                                                                                     *
# ***************************************************************************************************************************************/



class CallRust
{
    constructor()
    {
      return null;
    }
    
    oilApiGravity(specificGravity)
    {
      const { api } = require("./callRust.node");
      return api(specificGravity);
    }
    
    gammaDistributionFunction(a, x)
    {
      const { gdf } = require("./callRust.node");
      return gdf(a, x);
    }
}


(async function test()
{
  const cr = new  CallRust();
  cr.oilApiGravity(1.02);
  // value should be 8
  
  cr.gammaDistributionFunction(0.01, 0.02);
  // value should be 4.069063539262561
  
})();
