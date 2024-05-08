import os
import sys

import pandas as pd
from bs4 import BeautifulSoup
from selenium import webdriver
from selenium.webdriver.chrome.options import Options




def get_seliunm_driver():
    ch_options = Options()
    ch_options.add_argument('--headless')
    # ch_options.add_argument('--user-agent="Mozilla/5.0 (Windows Phone 10.0; Android 4.2.1; Microsoft; Lumia 640 XL '
    #                         'LTE) AppleWebKit/537.36 (KHTML, like Gecko) Chrome/42.0.2311.135 Mobile Safari/537.36 '
    #                         'Edge/12.10166"')
    ch_options.add_argument("user-agent=Mozilla/5.0 (Windows Phone 10.0; Android 4.2.1; Microsoft; Lumia 640 XL LTE) AppleWebKit/537.36 (KHTML, like Gecko) Chrome/42.0.2311.135 Mobile Safari/537.36 Edge/12.10166")
    ch_options.add_argument("--no-sandbox")
    ch_options.add_argument("--disable-gpu")
    ch_options.add_argument('blink-settings=imagesEnabled=false')
    ch_options.add_argument(f"--proxy-server={'http://127.0.0.1:7890'}")
    ff = webdriver.Chrome(executable_path='./chromedriver', options=ch_options)
    ff.set_page_load_timeout(20)
    ff.set_script_timeout(20)
    ff.set_window_size(360, 760)
    return ff


def process(url):
    # Start Selenium Browser (you need to download the driver of the corresponding browser)
    driver = get_seliunm_driver()
    # Load the page
    driver.get(url)

    # Load the page
    driver.implicitly_wait(10)

    # Get the source code of the page
    html = driver.page_source

    # Parse the page
    soup = BeautifulSoup(html, 'html.parser')

    if len(soup.find_all("div", "BtnGroup")[1].find_all('a')) == 1 and \
            soup.find_all("div", "BtnGroup")[1].find_all('a')[0].text == "Newer":
        # Locate the commit information element
        commit_elements = soup.find_all("li",
                                        "Box-row Box-row--focus-gray mt-0 d-flex js-commits-list-item js-navigation-item")

        for commit_element in commit_elements:
            info = commit_element.find("a", class_="Link--primary text-bold js-navigation-open markdown-title")
            commit_info = info.text.strip()
            commit_hash = "https://github.com/" + info["href"]
            print([commit_info, commit_hash])
            if ("ordering" in commit_info) or ("atomic" in commit_info):
                a = {}
                a['info'] = commit_info
                a['url'] = commit_hash
                result.append(a)
    else:
        # Locate the commit information element
        commit_elements = soup.find_all("li",
                                        "Box-row Box-row--focus-gray mt-0 d-flex js-commits-list-item js-navigation-item")

        for commit_element in commit_elements:
            info = commit_element.find("a", class_="Link--primary text-bold js-navigation-open markdown-title")
            commit_info = info.text.strip()
            commit_hash = "https://github.com/" + info["href"]
            driver.quit()
            print([commit_info, commit_hash])
            if ("ordering" in commit_info) or ("atomic" in commit_info):
                a = {}
                a['info'] = commit_info
                a['url'] = commit_hash
                result.append(a)
        btns = soup.find_all("div", "BtnGroup")[1].find_all('a')
        for page in btns:
            if page.text == "Older":
                driver.quit()
                print(page['href'])
                process(page['href'])


def generate_result(file_name):
    save_path = os.path.join(os.getcwd(), "result")
    if not os.path.exists(save_path):
        os.makedirs(save_path)

    file_path = os.path.join(save_path, file_name)
    df = pd.DataFrame(result, columns=['info', 'url'])
    df.to_csv(file_path, index=False)
    print("The CSV file has been saved at", file_path)


if __name__ == "__main__":
    if len(sys.argv) != 3:
        print("Usage: python main.py <URL> <CSV_FILE>")
        sys.exit(1)

    url = sys.argv[1]
    csv_file = sys.argv[2]
    # url ="https://github.com/crossbeam-rs/crossbeam/commits/master/"
    # csv_file ="servo.csv"

    result = []
    process(url)
    generate_result(csv_file)