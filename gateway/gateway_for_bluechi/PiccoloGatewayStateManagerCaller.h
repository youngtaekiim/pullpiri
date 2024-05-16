/*
 *
 * Copyright 2015 gRPC authors.
 *
 * Licensed under the Apache License, Version 2.0 (the "License");
 * you may not use this file except in compliance with the License.
 * You may obtain a copy of the License at
 *
 *     http://www.apache.org/licenses/LICENSE-2.0
 *
 * Unless required by applicable law or agreed to in writing, software
 * distributed under the License is distributed on an "AS IS" BASIS,
 * WITHOUT WARRANTIES OR CONDITIONS OF ANY KIND, either express or implied.
 * See the License for the specific language governing permissions and
 * limitations under the License.
 *
 */
/*
 * Copyright (c) 2024 LG Electronics Inc.
 * SPDX-License-Identifier: Apache-2.0
 */



#pragma once

#ifndef _PiccoloGatewayStateManagerCaller_
#define _PiccoloGatewayStateManagerCaller_

#include <iostream>
#include <memory>
#include <string>

#include "absl/flags/flag.h"
#include "absl/flags/parse.h"

#include <grpc/support/log.h>
#include <grpcpp/grpcpp.h>

#include "constants.grpc.pb.h"
#include "statemanager.grpc.pb.h"
#include "constants.pb.h"
#include "statemanager.pb.h"

/*
using grpc::Channel;
using grpc::ClientAsyncResponseReader;
using grpc::ClientContext;
using grpc::CompletionQueue;
using grpc::Status;
using statemanager::Connection;
using statemanager::SendResponse;
using statemanager::SendRequest;*/

class PiccoloGatewayStateManagerCaller {
 public:
  explicit PiccoloGatewayStateManagerCaller(std::shared_ptr<grpc::Channel> channel)
      : stub_(statemanager::Connection::NewStub(channel)) {}

  // Assembles the client's payload, sends it and presents the response back
  // from the server.
  bool Send(const std::string& key);

 private:
  // Out of the passed in Channel comes the stub, stored here, our view of the
  // server's exposed services.
  std::unique_ptr<statemanager::Connection::Stub> stub_;
};

#endif

