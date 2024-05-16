// SPDX-FileCopyrightText: Copyright 2024 LG Electronics Inc.
//
// SPDX-License-Identifier: Apache-2.0

#pragma once

#ifndef _PiccoloGatewayInfo_
#define _PiccoloGatewayInfo_

#include <string>

namespace PiccoloGatewayInfo
{
	std::string getDdsType(std::string s){
		if(s.compare(std::string("rt/piccolo/gear_state")) == 0){
			return "string";
		}else{
			return "no type info";
		}
	}
}

#endif
