FROM pypy:3.9-7.3.10

WORKDIR /heistream

COPY . /heistream
COPY ./requirements.txt /heistream/requirements.txt

RUN pypy -m pip install -U pip && pypy -m pip install --no-cache-dir --upgrade -r /heistream/requirements.txt

CMD ["pypy", "/heistream/heistream/main.py"]
