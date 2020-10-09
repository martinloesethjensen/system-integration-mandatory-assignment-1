FROM python:3.8
WORKDIR /code
COPY . . 
RUN pip install -r person.py
COPY src/ .
CMD [ "python", "./client.py" ]