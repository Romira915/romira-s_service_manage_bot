import time
from itertools import tee

from selenium import webdriver
from selenium.webdriver.common.by import By
from selenium.webdriver.common.keys import Keys

options = webdriver.FirefoxOptions()
options.headless = True
options.add_argument('--no-sandbox')
profile = webdriver.FirefoxProfile(
    "/home/ubuntu/.mozilla/firefox/k8q7oj2s.colab-cli")
driver = webdriver.Firefox(options=options, firefox_profile=profile)

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
time.sleep(10)
driver.save_screenshot("exe.png")

driver.quit()
