# action will be run in python3 container
FROM python:3@sha256:049247cbb7ce998685d780c9d5325781a7429bc6693bbfab86bcb8e36e7a059e
# copying requirements.txt and install the action dependencies
COPY requirements.txt /requirements.txt
RUN pip install -r /requirements.txt
# script.py is the file that will contain the codes that we want to run for this action.
COPY script.py /script.py
# we will just run our script.py as our docker entrypoint by python script.py
CMD ["python", "/script.py"]
