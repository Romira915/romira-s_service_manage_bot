import os
import time
from itertools import tee

import requests
from selenium import webdriver
from selenium.webdriver.common.by import By
from selenium.webdriver.common.keys import Keys

PROMPT_ENDPOINT = "https://k5vi72fcdo5u6gjqmuaqu5yoba0draxm.lambda-url.ap-northeast-1.on.aws/prompt"
TIMEOUT = 12

DRIVER_PATH = os.environ["DRIVER_PATH"]
FIREFOX_PROFILE = os.environ["FIREFOX_PROFILE"]

print("[INFO] run colab-cli")
options = webdriver.FirefoxOptions()
options.headless = True
options.add_argument('--no-sandbox')
profile = webdriver.FirefoxProfile(FIREFOX_PROFILE)

with webdriver.Firefox(options=options, firefox_profile=profile,
                       executable_path=DRIVER_PATH) as driver:
    driver.get(
        "https://colab.research.google.com/drive/16k_Z9PrVuxzOPi-R_5sIhgNYi227_wfH")
    time.sleep(3)
    driver.save_screenshot("google_colab.png")

    # すべて実行
    all_exe = driver.find_element(
        by=By.CSS_SELECTOR, value="#runtime-menu-button > div > div > div.goog-inline-block.goog-menu-button-caption")
    driver.maximize_window()
    # all_exe.send_keys(Keys.CONTROL, Keys.F9)
    all_exe.click()
    time.sleep(1)
    all_exe = driver.find_element(by=By.CSS_SELECTOR, value="#\:1y")
    all_exe.click()
    time.sleep(180)
    timeout_counter = 0
    while True:
        # あまりに遅い場合は異常終了とみなす
        if TIMEOUT <= timeout_counter:
            requests.post(PROMPT_ENDPOINT, json={"prompt": "~completed"})
            break
        timeout_counter += 1

        prompt = requests.get(PROMPT_ENDPOINT).json()["prompt"]
        if prompt == "~completed":
            break
        time.sleep(10)

    driver.save_screenshot("exe.png")
