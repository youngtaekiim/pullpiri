// SPDX-FileCopyrightText: Copyright 2024 LG Electronics Inc.
//
// SPDX-License-Identifier: Apache-2.0

#include "PiccoloGatewayManager.h"
#include "PiccoloGatewayServer.h"
#include "PiccoloGatewayStateManagerCaller.h"
#include <cstdlib>

#include <iostream>

int main(){
	std::cout << "Piccolo gateway start" << std::endl;

	const char* env_ip = std::getenv("HOST_IP");

	std::string ipAddress;
	if(env_ip == nullptr)
	{
		ipAddress = std::string("0.0.0.0");
	}else
	{
		ipAddress = std::string(env_ip);
	}

	PiccoloGatewayStateManagerCaller grpcCaller(
			grpc::CreateChannel(ipAddress.append(":47003"), grpc::InsecureChannelCredentials()));
	std::cout << "statemanager caller setup. " << ipAddress << std::endl;
	PiccoloGatewayManager pgm;
	pgm.setGrpcCaller(&grpcCaller);
	PiccoloGatewayServerImpl grpcServer;
        grpcServer.setManager(&pgm);

        grpcServer.Run();
	return 0;
}
