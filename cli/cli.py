import json
import os

import dotenv
import polars as pl
import requests


def load_mock_data():
    whiskey_types = json.load(open('../tests/mock_data.json'))
    for wt in whiskey_types:
        resp = requests.post('http://localhost:8000/whiskey_type/', json=wt)
        resp.is_ok()


def main():
    dotenv.load_dotenv(dotenv_path="../.env")

    resp = requests.get('http://localhost:8000/whiskey_type/').json()
    if len(resp['data']) == 0:
        load_mock_data()

    query = "SELECT id, name, description, annual_sale_in_liters FROM whiskey_type"

    df = pl.read_database_uri(query=query, uri=os.environ['DATABASE_URL'])
    print(df)


if __name__ == "__main__":
    main()
