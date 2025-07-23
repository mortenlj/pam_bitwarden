# action will be run in python3 container
FROM python:3@sha256:7175df81f9a313ee52286c94a5c35620d37afb31f9e05e47a3e058db84d53854
# copying requirements.txt and install the action dependencies
COPY requirements.txt /requirements.txt
RUN pip install -r /requirements.txt
# script.py is the file that will contain the codes that we want to run for this action.
COPY script.py /script.py
# we will just run our script.py as our docker entrypoint by python script.py
CMD ["python", "/script.py"]
