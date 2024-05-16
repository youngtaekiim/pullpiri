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

#ifndef _PICCOLOGATEWAYSERVER_
#define _PICCOLOGATEWAYSERVER_


#include <iostream>
#include <memory>
#include <string>
#include <thread>

#include "absl/flags/flag.h"
#include "absl/flags/parse.h"
#include "absl/strings/str_format.h"

#include <grpc/support/log.h>
#include <grpcpp/grpcpp.h>

#include "PiccoloGateway.grpc.pb.h"
#include "PiccoloGatewayManager.h"

class PiccoloGatewayServerImpl final {
 public:
  ~PiccoloGatewayServerImpl();
  void Run();

  void setManager(class PiccoloGatewayManager* m);

 private:
  class CallData {
   public:
    CallData(piccologatewaypackage::PiccoloGatewayService::AsyncService* service, grpc::ServerCompletionQueue* cq ,class PiccoloGatewayManager* manager);
    void Proceed();

   private:
    piccologatewaypackage::PiccoloGatewayService::AsyncService* service_;
    grpc::ServerCompletionQueue* cq_;
    class PiccoloGatewayManager* manager_;
    grpc::ServerContext ctx_;
    piccologatewaypackage::EventName eventName_;
    piccologatewaypackage::Reply reply_;
    grpc::ServerAsyncResponseWriter<piccologatewaypackage::Reply> responder_;
    enum CallStatus { CREATE, PROCESS, FINISH };
    CallStatus status_;
  };

  void HandleRpcs();
  std::unique_ptr<grpc::ServerCompletionQueue> cq_;
  piccologatewaypackage::PiccoloGatewayService::AsyncService service_;
  std::unique_ptr<grpc::Server> server_;
  class PiccoloGatewayManager* manager_;
};

#endif
