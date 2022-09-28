import os
import time
from itertools import tee

import requests
from selenium import webdriver
from selenium.webdriver.common.by import By
from selenium.webdriver.common.keys import Keys

PROMPT_ENDPOINT = "https://k5vi72fcdo5u6gjqmuaqu5yoba0draxm.lambda-url.ap-northeast-1.on.aws/prompt"
TIMEOUT = 20

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
    runtime = driver.find_element(
        by=By.CSS_SELECTOR, value="#runtime-menu-button > div > div > div.goog-inline-block.goog-menu-button-caption")
    # driver.maximize_window()
    # all_exe.send_keys(Keys.CONTROL, Keys.F9)
    runtime.click()
    time.sleep(1)
    all_exe = driver.find_element(by=By.CSS_SELECTOR, value="#\:1y")
    all_exe.click()

    timeout_counter = 0
    wait_for_secs = 2
    while timeout_counter < 60 / wait_for_secs:
        time.sleep(wait_for_secs)
        try:
            dialog = driver.find_element(
                by=By.CSS_SELECTOR, value="body > colab-dialog > paper-dialog")
            break
        except:
            pass
        timeout_counter += 1
    print("dialog")

    timeout_counter = 0
    wait_for_secs = 2
    while timeout_counter < 60 / wait_for_secs:
        time.sleep(wait_for_secs)
        try:
            connect_gdrive = driver.find_element(
                by=By.CSS_SELECTOR, value="#ok")
            connect_gdrive.click()
            print("connect gdrive")
            break
        except:
            pass
        timeout_counter += 1

    time.sleep(3)

    driver.switch_to.window(driver.window_handles[-1])

    select_account = driver.find_element(by=By.CSS_SELECTOR, value=".d2laFc")
    select_account.click()

    timeout_counter = 0
    wait_for_secs = 2
    while timeout_counter < 60 / wait_for_secs:
        time.sleep(wait_for_secs)
        try:
            submit_approve_access = driver.find_element(
                by=By.CSS_SELECTOR, value="#submit_approve_access > div:nth-child(1) > button:nth-child(1) > span:nth-child(4)")
            submit_approve_access.click()
            break
        except:
            pass
        timeout_counter += 1

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

    driver.switch_to.window(driver.window_handles[-1])

    # colabから接続解除
    # runtime = driver.find_element(
    #     by=By.CSS_SELECTOR, value="#runtime-menu-button > div > div > div.goog-inline-block.goog-menu-button-caption")
    # runtime.click()
    # print("runtime")
    # time.sleep(1)

    # disconnect = driver.find_element(
    #     by=By.CSS_SELECTOR, value="#\:27")
    # disconnect.click()
    # print("disconnect")
    # time.sleep(1)

    # dialog = driver.find_element(
    #     by=By.CSS_SELECTOR, value="body > colab-dialog > paper-dialog > div.content-area > div")
    # dialog.click()
    # print("dialog")
    # time.sleep(1)

    # yes = driver.find_element(
    #     by=By.CSS_SELECTOR, value="#ok")
    # yes.click()
    # print("yes")
    # time.sleep(1)

    driver.save_screenshot("exe.png")
