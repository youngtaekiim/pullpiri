# SPDX-License-Identifier: Apache-2.0

import sys
import os
import time

if __name__ == '__main__':
    while True:
        ver = os.environ.get('VERSION')
        print(f"version: {ver}")
        time.sleep(10)
